use std::fmt;

struct Profile {
    name:String,
    age:u8,
    sex:String,
    country:String
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "Name =  {}\nAge = {}\nGender = {}\nCountry = {}", self.name, self.age, self.sex, self.country)
    }
}

fn main() {
    let _var1 = 1;
    let _var2:u8 = 251;
    let mut _var3:u32;
    _var3 = 99999;
    _var3 = 999;

    println!("Var 1 = {}, \nVar 2 = {}, \nVar 3 = {}", _var1, _var2, _var3);

    let _flag = 3 == 3;
    println!("Result is {}", _flag);

    if 9 > 5{
        println!("9 is Greater than 5")
    }
    else
    {
        println!("5 is Greater than 9")
    }

    let _flag_result = if _flag {"Flag is True"} else {"Flag is False"};
    println!("{}", _flag_result);

    while _var3 >= 900{
        print!("{}-", _var3);
        _var3 = _var3 - 1;
    }

    let square_res = squarer(32);
    println!("\nSquare of 32 is = {}", square_res);

    let mul = multiplyer(32, 64);
    println!("32 x 64 = {}", mul);

    let profile = Profile{
        name:"Noman Nasir Minhas".to_string(),
        age:25,
        sex:"Male".to_string(),
        country:"Pakistan".to_string()
    };

    println!("Your Profile is \n{}", profile);

    let mut name = String::new();
    let mut age = String::new();
    let mut gender = String::new();
    let mut country = String::new();

    println!("\nPlease Enter Your Name");
    std::io::stdin().read_line(&mut name).expect("Failed to read Line");

    println!("\nPlease Enter Your Age");
    std::io::stdin().read_line(&mut age).expect("Failed to read Age");

    println!("\nPlease Enter Your Gender");
    std::io::stdin().read_line(&mut gender).expect("Failed to read Gender");

    println!("\nPlease Enter Your Country");
    std::io::stdin().read_line(&mut country).expect("Failed to read Country");

    let new_profile = Profile{
        name: name.trim().to_string(),
        age: age.trim().parse().expect("Failed to Parse the Age Entered"),
        sex: gender.trim().to_string(),
        country: country.trim().to_string()
    };

    println!("\nYour New Entered Profile is \n---------------------------------\n{}", new_profile);
}

fn squarer(num:i32) -> i32 {
    num*num
}

fn multiplyer(num1:i32, num2:i32) -> i32{
    num1*num2
}