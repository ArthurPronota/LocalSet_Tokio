use tokio::task ;
use std::rc::Rc ;
use std::cell::RefCell ;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // set (набор) задач которые являются выполненными на текущем потоке
    let loc = task::LocalSet::new() ;   // Возвращает новый set локальеых задач
    
    // Запускает future до завершения local set возвращая его вывод
    loc.run_until(
    async move {
        // счётчик
        let counter = Rc::new(RefCell::new(0)) ;

        // все обработчики порождённых tokio tasks
        let mut handls = vec![] ;

        for i in 0..5 {
            // клонирование счётчика
            let counter = counter.clone() ;
            handls.push(    // Заполнение обработчика
                task::spawn_local( // Порождает !Send future не текущем LocalSet
                    async move {
                        *counter
                            .borrow_mut() // Изменяемое заимствование обёрнутого значения
                            += 1 ;
                        println!("Task: {} is done", i) ;
                    }
                )
            );
        }

        for h in handls {
            h
                .await // дождаться окончаниявыполнения задачи
                .unwrap()
                ;
        }

        println!("Итоговое значение счётчика: {}", *counter.borrow()) ; // Out: Итоговое значение счётчика: 5
    })
    .await ;
}
