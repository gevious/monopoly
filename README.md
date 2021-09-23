# monopoly
Monopoly clone, to help visually impaired keep track of resourcesG

# Roadmap 
- [x] Infinite loop asking for dice roll
- [x] Entering num players, and record name of each player
- [x] Show summary of all players at start of each turn (money, position, assets)
- [x] Each player has a turn. Dice number moves player to correct position
- [x] Game displays position player lands on
- [x] Game displays options a player has on each position
- [x] Player is charged for events like income tax
- [x] Player can receive money from bank
- [x] Player can pay bank money
- [x] Player automatically buys street (if has enough money)
- [x] Player is charged rent for basic streets
- [x] Player is sent to jail
- [x] Card event: Pay/receive money from bank
- [x] Card event: Move to different square
- [x] Card event: get out of jail card
- [x] Player can leave jail (using pay $50, or using get-out-of-jail card)
- [x] Cards shuffled at start of the game
- [x] Player is charged rent for water/electricity
- [x] Player is charged rent for multiple stations 
- [x] Player is charged rent for streets if player owns all streets
- [x] Player can choose to auction street (if applicable)
- [x] Player can sell street to another player
- [ ] Player can buy house (calculates applicable streets)
- [ ] Player is charged rent for streets for x number of houses
- [ ] Player can buy hotel (calculates applicable streets)
- [ ] Player is charged rent for streets for hotel
- [ ] Player can mortgage street
- [ ] Player can unmortgage street
- [ ] Player must use 2 dice
- [ ] 3 double rolls sends player to jail
- [ ] while in jail, a double roll frees player
- [ ] while in jail, player has a choice to pay to leave jail
- [ ] utility charges player for cumulative double rolls (not just latest roll)
- [ ] Card event: make repairs on houses/hotels
- [ ] Card event: pay players money for birthday
- [ ] Player cannot go into negative cash
- [ ] If player cannot pay rent, enforce selling assets

# S3 upload
Currently, I hacked the s3 upload to use the AWS CLI. Credentials are stored locally, in the 'monopoly' profile
