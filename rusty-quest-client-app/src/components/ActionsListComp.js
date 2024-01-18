import React, { useState } from 'react';
import {useGameState} from '../contexts/GameStateContext';
import * as ApiService from '../services/apiService';
import { useIpAddress } from '../contexts/IpAddressContext';


const ActionsListComp = ({ actionsListInfo}) => {
    const [selectedAction, setSelectedAction] = useState(null);
    const {gameState, setGameState } = useGameState();
    const {ipAddress, setIpAddress} = useIpAddress();
    
    const handleActionSelect = (action) => {
        setSelectedAction(action);
    };

    let activePlayerActions = [];
    if(gameState && gameState.active_player_id && gameState.actions_map) {
        activePlayerActions = gameState.actions_map[gameState.active_player_id] || [];
        console.log(activePlayerActions); // Check the value
    }


    const sendAction = () => {
        if (selectedAction) {
            // Implement what happens when an action is sent            
            ApiService.sendPlayerAction(ipAddress,gameState.id,gameState.active_player_id,selectedAction)
                .then(data => checkResponse(data))
                .catch(error => console.error('Error fetching actions description:', error));
            console.log('Sending action:', selectedAction);
        }
    };

    const checkResponse = (data) => {
        /// TODO: check for an error or other response from a sent action
    };

    return (
        <div>
            <div style={{ maxHeight: '200px', overflowY: 'scroll', border: '1px solid black', padding: '10px' }}>
                {activePlayerActions && activePlayerActions.actions ? 
                activePlayerActions.actions.map((action, index) => (
                     <div key={index} onClick={() => handleActionSelect(action)} style={{ cursor: 'pointer', backgroundColor: selectedAction === action ? '#ddd' : '' }}>
                         <p><strong>Action:</strong> {action}</p>
                         <p>DESCRIPTION: {actionsListInfo.actions[action].description}</p>
                         <p>ENDPOINT: {actionsListInfo.actions[action].endpoint}</p>
                     </div>
                ))
                :
                <div></div>
            }
            </div>
        <button onClick={sendAction}>Send Action</button>   
      
      </div>
    );
  };

  
export default ActionsListComp;