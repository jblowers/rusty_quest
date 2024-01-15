import React, { useState } from 'react';


const ActionsListComp = ({ gameState, actionsListInfo}) => {
    const [selectedAction, setSelectedAction] = useState(null);
    
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

          console.log('Sending action:', selectedAction);
        }
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