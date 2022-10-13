fn main() {
    let mut age = 13;
    println!("{}", age);
    age = 32;
    println!("{}", age);

    // # vecteur
    let date = vec![2021, 2022];
    print(&date);
    print(&date);
}

//  Vecteur
fn print(dn: &Vec<i32>) {
    println!("date is {:?}", dn);
}

// regrouper des champs
// structure = no comportement
// class =  comportent
