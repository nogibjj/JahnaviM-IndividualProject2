# Jahnavi Maddhuri: Individual Project 2
In this project I create a build system for rust scripts to perform ETL on a specific dataset and demonstrate CRUD operations using SQLite. I have also made my main script interactive so users can enter custom queries. The data set I use is a bad drivers dataset available in github that has average statistics by state of drivers in fatal car collisions. This system creates a base feedback loop every time I update my project.

## CI/CD Badge
[![CICD](https://github.com/nogibjj/JahnaviM-IndividualProject2/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/JahnaviM-IndividualProject2/actions/workflows/cicd.yml)

## Instructions for Use
To use this repository to generate the analysis follow the steps below:
1. Start by cloning the repository. 
2. Setup the rust script by navigating to src and running `cargo build --release`.
4. Run the rust script by then running `cargo run ETL` or `cargo run Q <your custom query>`.
      The first option will perform ETL on the data set to Load it as a csv file and database in the data directory.
      The second option allows you to run your own custom query.

## Sample Script CRUD Operations
Create:
<img width="1719" alt="image" src="https://github.com/user-attachments/assets/dbedb0ed-e89b-4824-aae5-07deb4e71799">

Read:
<img width="1567" alt="image" src="https://github.com/user-attachments/assets/090b2b52-1018-4119-9002-af188c22cf93">

Update:
<img width="1572" alt="image" src="https://github.com/user-attachments/assets/78aa4679-d7be-4e49-807c-d00a89167bb7">

Delete:
<img width="1627" alt="image" src="https://github.com/user-attachments/assets/5b319f4f-4d2d-4f21-8ae4-24efe8ca7558">

## Copilot Use
I used copilot as a starting point to converting my python extract and transform functions to rust. Here is an example prompt and output I used:
<img width="992" alt="image" src="https://github.com/user-attachments/assets/510a29a9-a816-4e40-ab84-5fd3ca0fb7a4">
I also often used copilot to help with debugging and learning the basic syntax of Rust. Once the AI would explain to me once why my code needed to change for a specific error, I was able to apply that information to future code I developed. I also enjoyed using the suggestions while I was coding. This made the process faster in some ways since I did not need to manually type out everything, but I also found myself prone to the resulting code deviating from the purpose of the script. It was important for me to be conscious with the suggestions I decide to use/not in this lab.
