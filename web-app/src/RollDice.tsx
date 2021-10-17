import React, { Component } from 'react';
import { DiceRoll} from './types/types';

export default class RollDice extends Component<DiceRoll> {

	render() {
		return(
		<div id="dice_roll">
		    Roll Dice 
		</div>
		)
	}
}
