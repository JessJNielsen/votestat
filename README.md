# VoteStat

Vote Statistics and Analysis

With this tool you can download data from KMD.  
Then you can either do simple analysis or export it for more detailed analysis.

## Structure

The project has been split into smaller library crates:

- `dal` - Database and Entities, anything that uses `sqlx`. Includes SQL migrations.
- `scraping` - Scraping function in VoteStat CLI. Includes KMD provider logic.
- `utils` - Various shared utilities. Includes the `Context`
- `votestat` - The main Votestat CLI application

## Terms

| Danish term | English term          | Code term      |
|-------------|-----------------------|----------------|
| Storkreds   | Voting super district | super_district |
| Kreds       | Voting district       | district       |
| Valgsted    | Voting location       | voting_center  |
| Stemme      | Vote                  | vote           |