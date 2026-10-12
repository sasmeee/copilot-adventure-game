use std::io::{self, Write};

// Read a numbered choice and keep asking until the player enters a valid one.
fn read_choice(prompt: &str, options: &[&str]) -> usize {
    loop {
        println!("\n{prompt}");
        for (index, option) in options.iter().enumerate() {
            println!("  {}. {option}", index + 1);
        }

        print!("> ");
        io::stdout().flush().expect("Could not flush the prompt");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read your choice");

        match input.trim().parse::<usize>() {
            Ok(choice) if (1..=options.len()).contains(&choice) => return choice,
            _ => println!("Please enter a number from 1 to {}.", options.len()),
        }
    }
}

fn play_adventure() {
    println!("\n=== The Lantern of Moonfall ===");
    println!("You wake beside an ancient forest road with a silver lantern in your hand.");

    let first_choice = read_choice(
        "The road divides. Where will you go?",
        &["Follow the lantern-lit trail", "Enter the echoing cave"],
    );

    if first_choice == 1 {
        println!("The lantern guides you to a mossy bridge above a rushing river.");

        let bridge_choice = read_choice(
            "A frightened fox blocks the bridge. What do you do?",
            &["Share your food with the fox", "Cross quickly and leave it behind"],
        );

        if bridge_choice == 1 {
            println!("The fox bows, then reveals a hidden path to a moonlit garden.");
            println!("Ending: The garden keeper welcomes you as a friend of the forest!");
        } else {
            println!("The bridge shakes as you run. You reach the far side, but the lantern goes dark.");
            println!("Ending: You escape safely, though the forest's secret remains undiscovered.");
        }
    } else {
        println!("Inside the cave, you find a sleeping dragon guarding a golden door.");

        let dragon_choice = read_choice(
            "How will you approach the dragon?",
            &["Sing a gentle song", "Take the treasure and run"],
        );

        if dragon_choice == 1 {
            println!("Your song wakes the dragon peacefully. It opens the golden door for you.");
            println!("Ending: Beyond the door, you discover a library of forgotten magic.");
        } else {
            println!("The dragon wakes with a thunderous roar before you reach the treasure.");
            println!("Ending: You flee the cave with a memorable lesson about borrowing dragons' gold.");
        }
    }
}

fn main() {
    println!("Welcome, traveler!");

    // Restarting the outer loop lets the player begin a fresh story without relaunching.
    loop {
        play_adventure();

        let play_again = read_choice("Would you like to play again?", &["Yes", "No"]);
        if play_again == 2 {
            println!("Thanks for playing. Farewell!");
            break;
        }
    }
}
