import React from 'react';
import './App.css';
import { DiceRoll} from './types/types';
import axios from 'axios';

function RollDice() {

    const [dice1, setDice1] = React.useState<number>(0);
    const [dice2, setDice2] = React.useState<number>(0);

    const sendDice = (e: React.MouseEvent<HTMLButtonElement>) => {
		e.preventDefault();
		if (dice1 < 1 || dice1 > 6 || dice2 < 1 || dice2 > 6) {
			alert("Dice must be between 1 and 6");
			return;
		}

		const data:DiceRoll = {dice1: dice1, dice2: dice2};
		axios({
			url: "http://192.168.0.119:8000/roll-dice",
			method: "post",
			data: data
		}).then(res => {
			// UPDATE site with data
		}, err => {
			console.log("Error in request");
		});
	}

	return (
		<div id="dice_roll">
			RollDice
			<input id="dice1" type="number" value={dice1} required
				   placeholder="Enter value of first dice"
 				   onChange={(e) => setDice1(+e.target.value)}></input>
			<input id="dice2" type="number" value={dice2} required
				   placeholder="Enter value of second dice"
 				   onChange={(e) => setDice2(+e.target.value)}></input>
			<button onClick={sendDice}>Go!</button>
		</div>
	)
}

function App() {

  return (
    <div className="Monopoly">
		<RollDice></RollDice>
		<div id="actions">
		</div>

		<div id="journal">
		</div>

		<div id="costs">
			<h1>Costs</h1>
			<div className="street">
				<h2>Mediterranean Avenue</h2>
				<dl>
					<dt>Price</dt>
					<dd>$40</dd>
					<dt>Rent</dt>
					<dd>2</dd>
					<dt>Rent (colorset)</dt>
					<dd>10</dd>
					<dt>Rent 1 house</dt>
					<dd>10</dd>
					<dt>Rent 2 houses</dt>
					<dd>10</dd>
					<dt>Rent 3 houses</dt>
					<dd>10</dd>
					<dt>Rent Hotel</dt>
					<dd>10</dd>
					<dt>Mortgage</dt>
					<dd>10</dd>
					<dt>Mortgage Repayment</dt>
					<dd>10</dd>

				</dl>
			</div>
		</div>
    </div>
  );
}

export default App;
