# VoteStat
Vote Statistics and Analysis

With this tool you can download data from KMD.  
Then you can either do simple analysis or export it for more detailed analysis.

## Structure
The project has been split into smaller library crates:

 - `dal` - Database and Models, anything that uses `sqlx`
 - `scraping` - Scraping function in VoteStat cli. Includes KMD logic.
 - `votestat` - The main CLI application

## Terms

| Danish term | English term          | Code term      |
|-------------|-----------------------|----------------|
| Storkreds   | Voting super district | super_district |
| Kreds       | Voting district       | district       |
| Valgsted    | Voting location       | voting_center  |
| Stemme      | Vote                  | vote           |