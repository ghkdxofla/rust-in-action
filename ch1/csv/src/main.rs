fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record // 동적으로 확장할 수 있는 컬렉션 타입. '_'는 해당 요소의 타입을 추론하라고 지시하는 역할.
            .split(',')
            .map(|field| field.trim()) // 익명함수(Lambda, closure) 사용
            .collect(); // field 집합을 하나로 합친다
        if cfg!(debug_assertions) { // 조건부 컴파일. 배포 버전을 만들 때는 포함되지 않음. cfg는 환경 설정을 검사함
            eprintln!("debug: {:?} -> {:?}", record, fields); // 표준 오류(stderr)로 내용 출력
        }

        let name = fields[0];
        // 부동 소수점 수로 분석. let Ok(length)는 부동 소숫점 수로 해석 시도 후
        // 성공하면 length에 값을 할당하라는 방법
        // 실패 시, Err(T)가 반환되나, 이를 처리하지 않음을 의미한다
        if let Ok(length) = fields[1].parse::<f32>() { // f32로 type annotation
            println!("{}, {}cm", name, length); // 표준 출력(stdout)으로 내용 출력
        }
    }
}

// cargo run 결과 (1.08s)
/*
debug: "    Little penguin,33" -> ["Little penguin", "33"]
Little penguin, 33cm
debug: "    Yellow-eyed penguin,65" -> ["Yellow-eyed penguin", "65"]
Yellow-eyed penguin, 65cm
debug: "    Fiordland penguin,60" -> ["Fiordland penguin", "60"]
Fiordland penguin, 60cm
debug: "    Invalid,data" -> ["Invalid", "data"]
*/

// cargo run --release 결과 (0.80s) 원래는 이게 더 컴파일 시 오래걸리지만, 실행이 더 빠르다
/*
Little penguin, 33cm
Yellow-eyed penguin, 65cm
Fiordland penguin, 60cm
*/
