/* Zad 1
fn main() {
    let a = 2015;
    if a%4 != 0{
        println!("Jest to rok przestępny");
    }
}
*/
/* Zad 2
fn main() {
    let year = 2012;
    let month = 4;
    let a = 2015;
    if year%4 != 0{
        println!("Jest to rok przestępny");
        if month == 2{
            println!("W tym roku {} dany miesiąc ma 29 dni", year);
        }
        else if month < 7 && month%2 == 0 {
            println!("W tym roku {} dany miesiąc ma 30 dni", year);

        }
        else if month < 7 && month%2 != 0 {
            println!("W tym roku {} dany miesiąc ma 29 dni", year);
        }
        else if month >= 7 && month%2 == 0{
            println!("W tym roku {} dany miesiąc ma 31 dni", year);
        }
        else if month >= 7 && month%2 != 0{
            println!("W tym roku {} dany miesiąc ma 30 dni", year);
        }
    }else {
        println!("Jest to rok nie przestępny");
        if month == 2{
            println!("W tym roku {} dany miesiąc ma 28 dni", year);
        }
        else if month < 7 && month%2 == 0 {
            println!("W tym roku {} dany miesiąc ma 30 dni", year);

        }
        else if month < 7 && month%2 != 0 {
            println!("W tym roku {} dany miesiąc ma 29 dni", year);
        }
        else if month >= 7 && month%2 == 0{
            println!("W tym roku {} dany miesiąc ma 31 dni", year);
        }
        else if month >= 7 && month%2 != 0{
            println!("W tym roku {} dany miesiąc ma 30 dni", year);
        }
    }
}
*/
/* Zad 3
fn main(){
    let mut temp = 31.0;
    temp = temp*(9.0/5.0) + 32.0;
    println!("Thats {} in Farenheit", temp);
}
*/
/* Zad 4
fn main(){
    let mut temp = 87.8;
    temp = ((temp-32.0)*5.0/(9.0));
    println!("Thats {} in Celcius", temp);
}
*/
/* zad 5
fn main() {
    let mut h1 = 12;
    let h2 = 10;
    let mut m1 = 12;
    let m2 = 32;
    let mut s1 = 44;
    let s2 = 55;
    h1 -= h2;
    if m1 < m2 {
        m1 += 60;
        m1 -= m2;
        h1 -= 1; 
    } else {
        m1 -= m2;
    }
    if s1 < s2 {
        s1 += 60;
        s1 -= s2;
        m1 -= 1; 
    } else {
        s1 -= s2;
    }
    println!("Różnica czasu wynosi: {}:{}:{}", h1, m1, s1);
}
*/
/* Zad 6
fn silnia(n : i8) -> i8{
    if n <= 1{
        return n;
    }else {
        return n * silnia(n-1);
    }
}
fn main(){
    let mut n = 5;
    let result = silnia(n);
    println!("Wynik silni: {}", result);
}
*/
/* Zad 7
fn main() {
    let mut a = 234;
    while(a>0){
        println!("{}", a%10);   
        a/=10;
    }
}
*/
/* Zad 8
fn main() {
    let mut a = 234;
    let mut b = 0;
    while(a>0){
        b += a%10;   
        a/=10;
    }
    println!("Wynik: {}", b);
}
*/


fn main() {
    println!("Podaj wartość dla n:");
    let mut n = 10

    for a in 1..=n {
        for b in a+1..=n {
            for c in b+1..=n {
                if a*a + b*b == c*c {
                    println!("{}^2 + {}^2 = {}^2", a, b, c);
                }
            }
        }
    }
}