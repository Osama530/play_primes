// fn prim_serise(min: i32, max: i32) -> Vec<i32> {
//     let mut series: Vec<i32> = Vec::new(); //empty vector initialization
//     let mut count = 0;

//     for num in min..max {
//         count = 0;
//         for i in 2..num {
//             if num % i == 0 {
//                 count += 1;
//                 break;
//             }
//         }
//         if count == 0 && num != 1 {
//             series.push(num);
//         }
//     }
//     series
// }

// fn main() {
//     // let primes = prim_serise(2, 100);
//     // println!("{:?}",primes);

//     let primes = prim_serise(2, 100);
//     for i in primes.iter() {
//         println!("{} {:b}", i, i);
//     }
// }

// fn is_prime (num:i32)->bool{
//     let mut flag = 1;
//     if num <= 1 {
//         false
//     }
//     else {
//         for i in 2..num/2+1 {
//             if num%i == 0 {
//                 flag = 0;
//                 break;
//             }
//         }
//         if flag==0 {
//             false
//         }
//         else {
//             true
//         }
//     }
// }
// fn main() {

//     // let primes = prim_serise(2, 100);
//     // println!("{:?}",primes);

//     let prime = is_prime(4);
//     println!("{}",prime);

// }
#[derive(Clone)]
#[derive(Debug)]
struct Students {
    name: String,
    age: String,
    education: String,
    timing: String
}
impl Students {

    fn get_name(student: Students)-> String {
        student.name.to_string()
    }
    fn get_timing(student: Students)-> String {
        student.timing.to_string()
    }
    fn get_education(student: Students)-> String {
        student.education.to_string()
    }
}
//constructer function
impl Students {
    fn new(name: String, age: String)-> Students{
        Students {
            name,
            age,
            education: "Intermediate".to_string(),
            timing: "Morning".to_string()
        }
    }
}
fn main(){
    let student_01 = Students::new("osama qamar".to_string(), "27".to_string());
    let student_02 = Students::new("abdullah Alam".to_string(), "16".to_string());
    //println!("{:?}",student_01);
    let student_name_01 = Students::get_name(student_01.clone());
    let student_name_02 =Students::get_name(student_02.clone());
    println!("{}",student_name_01);
    println!("{}",Students::get_education(student_01));
    println!("{}",student_name_02);
    println!("{}",Students::get_education(student_02));
    

}