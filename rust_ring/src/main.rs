#[derive(Debug)]
pub struct RingBuffer {
    buffer: Vec<Option<u8>>, // Хранилище данных
    capacity: usize,         // Максимальная вместимость буфера
    head: usize,             // Индекс для чтения
    tail: usize,             // Индекс для записи
    size: usize,             // Текущее количество элементов
}

// Определяем типизированную ошибку для случая, когда нет места
#[derive(Debug, PartialEq)]
pub enum RingBufferError {
    NoSpaceLeft, // Ошибка, когда в буфере нет места для записи
}
impl RingBuffer {
    // Создаём новый буфер заданного размера
    pub fn new(capacity: usize) -> Self {
        // Проверяем, что размер буфера положительный
        assert!(capacity > 0, "Буффер не положительный");

        RingBuffer {
            buffer: vec![None; capacity], // Инициализируем пустыми значениями
            capacity,                     // Сохраняем ёмкость
            head: 0,                      // Начинаем с индекса 0
            tail: 0,                      // Начинаем с индекса 0
            size: 0,                      // Начальный размер - 0
        }
    }

    // Проверка на пустоту
    pub fn is_empty(&self) -> bool {
        self.size == 0 // Если размер 0 - буфер пуст
    }

    // Проверка на заполненность
    pub fn is_full(&self) -> bool {
        self.size == self.capacity // Если размер равен ёмкости - буфер полон
    }

    // Текущее количество элементов
    pub fn len(&self) -> usize {
        self.size // Просто возвращаем размер
    }

    // Запись одного элемента
    pub fn push(&mut self, value: u8) -> Result<usize, RingBufferError> {
        if self.is_full() {
            return Err(RingBufferError::NoSpaceLeft); // Ошибка если полон
        }

        self.buffer[self.tail] = Some(value); // Записываем значение
        self.tail = (self.tail + 1) % self.capacity; // Перемещаем хвост с закольцовыванием
        self.size += 1; // Увеличиваем размер
        Ok(1) // Возвращаем количество записанных байт (1)
    }

    // Чтение одного элемента
    pub fn pop(&mut self) -> Option<u8> {
        if self.is_empty() {
            return None; // Возвращаем None если пуст
        }

        let value = self.buffer[self.head].take(); // Забираем значение из головы
        self.head = (self.head + 1) % self.capacity; // Перемещаем голову
        self.size -= 1; // Уменьшаем размер
        value // Возвращаем значение
    }

    // Запись нескольких элементов
    pub fn extend(&mut self, data: &[u8]) -> Result<usize, RingBufferError> {
        if data.is_empty() {
            return Ok(0); // Ничего не записываем, если входной срез пуст
        }

        let mut count = 0;
        for &byte in data {
            if self.is_full() {
                // Если после записи хотя бы одного элемента буфер заполнился
                return if count > 0 {
                    Ok(count) // Если что-то записали - возвращаем количество
                } else {
                    Err(RingBufferError::NoSpaceLeft) // Если ничего не записали - ошибка
                };
            }

            self.buffer[self.tail] = Some(byte);
            self.tail = (self.tail + 1) % self.capacity;
            self.size += 1;
            count += 1;
        }

        Ok(count) // Возвращаем количество успешно записанных байт
    }

    // Чтение нескольких элементов
    pub fn drain(&mut self, count: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(count.min(self.size)); // Оптимизация: резервируем место
        for _ in 0..count {
            match self.pop() {
                Some(byte) => result.push(byte),
                None => break, // Прерываем если буфер пуст
            }
        }
        result
    }
}
fn main() {
    let mut buffer = RingBuffer::new(3);

    // Заполнение буфера
    println!("Запись 10: {:?}", buffer.push(10)); // Ok(1)
    println!("Запись 20: {:?}", buffer.push(20)); // Ok(1)
    println!("Запись 30: {:?}", buffer.push(30)); // Ok(1)

    // Попытка переполнения
    println!("Попытка записи 40: {:?}", buffer.push(40)); // Err(NoSpaceLeft)

    // Чтение данных
    println!("Чтение: {:?}", buffer.pop()); // Some(10)
    println!("Чтение: {:?}", buffer.pop()); // Some(20)

    // Запись с использованием extend
    println!("Запись [4,5]: {:?}", buffer.extend(&[4, 5])); // Ok(2)

    // Чтение нескольких элементов
    println!("Чтение 3 элементов: {:?}", buffer.drain(3)); // [30, 4, 5]
}
#[test]
fn test_extend() {
    let mut rb = RingBuffer::new(3);
    assert_eq!(rb.extend(&[1, 2, 3]), Ok(3)); // Проверяем успешную запись
    assert_eq!(rb.extend(&[4]), Err(RingBufferError::NoSpaceLeft)); // Проверяем ошибку

    let mut rb = RingBuffer::new(3);
    assert_eq!(rb.extend(&[1, 2]), Ok(2));
    assert_eq!(rb.extend(&[3, 4, 5, 6]), Ok(1)); // Проверяем частичную запись
    assert_eq!(rb.drain(3), vec![1, 2, 3]);
}
