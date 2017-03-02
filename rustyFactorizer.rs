fn main()
{
    use std::io;

    loop
    {
        println!("Enter your number here: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input: i32 = input
            .trim()
            .parse()
            .expect("Non-integer input detected. Exiting program.");

        factorize(input);
    }
}

fn is_prime(n : i32) -> bool
{
    if n == 2 || n == 3 {return true;}
    for i in 2..(n as f64).sqrt().ceil() as i32 + 1
    {
        if n % i == 0
        {
            return false;
        }
    }
    return true;
}

fn factorize(mut z : i32)
{
    if z < 0
    {
        print!("{}, ", -1);
        z = z * -1;
    } else if z == 0
    {
        println!("No factors");
        return;
    }
    loop
    {
        if is_prime(z)
        {
            println!("{}", z);
            break;
        } else
        {
            let mut i : i32 = 1;
            loop
            {
                i = i + 1;
                if is_prime(i) && z % i == 0
                {
                    print!("{}, ", i);
                    z = z / i;
                    break;
                }
            }
        }
    }
}
