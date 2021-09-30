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
- [x] Player can mortgage street
- [x] Player can unmortgage street
- [x] Mortgaged properties don't collect rent
- [x] For dialog selections, press 'q' to go back to main menu
- [x] Show color set next to houses in summary (and on web page)
- [x] Player can buy house (calculates applicable streets)
- [x] Player is charged rent for streets for x number of houses
- [x] Player can buy hotel (calculates applicable streets)
- [x] Player is charged rent for hotel
- [x] Unit test for sell property (ensure house is removed, and money received)
- [x] Player must use 2 dice
- [x] 3 double rolls sends player to jail
- [x] while in jail, a double roll frees player
- [x] while in jail, player has a choice to pay to leave jail
- [x] utility charges player for cumulative double rolls (not just latest roll)
- [ ] Check if player can afford house
- [ ] Confirm with player, before buying building, and publish cost
- [ ] If player cannot pay rent (tax etc), enforce selling assets
- [ ] Card event: make repairs on houses/hotels
- [ ] Card event: pay players money for birthday

# S3 upload
Currently, I hacked the s3 upload to use the AWS CLI. Credentials are stored locally, in the 'monopoly' profile
