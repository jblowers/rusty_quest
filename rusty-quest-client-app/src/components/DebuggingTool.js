import React, { useState } from 'react';

const DEFAULT_URL = "http://192.168.0.134:3030"; // Replace with your server's IP

function DebuggingTool({ ipAddress, selectedGameId }) {
    const [endpoint, setEndpoint] = useState('/game_state/0');
    const [responseLog, setResponseLog] = useState('');
    const [playerName, setPlayerName] = useState(`player_${selectedGameId}`);

    // const [gameId, setGameId] = useState(selectedGameId);

    const clearLogs = () => {
        setResponseLog('');
    }

    const sendRequest = () => {
        const validIpAddress = typeof ipAddress === 'string' ? ipAddress : DEFAULT_URL;
        const fullUrl = validIpAddress + endpoint;
        const typ = typeof validIpAddress;
        const requestLog = `Request type: ${typ}\tRequest: ${fullUrl}\n`;

        fetch(fullUrl)
            .then(response => {
                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }
                return response.json();
            })
            .then(data => {
                const formattedResponse = JSON.stringify(data, null, 2);
                setResponseLog(prevLog => requestLog + "Response:\n" + formattedResponse + "\n\n" + prevLog);
            })
            .catch(error => {
                setResponseLog(prevLog => requestLog + "Error: " + error + "\n\n" + prevLog);
            });
        };

    const onNewPlayerDebug = () => {
        const endpoint = `/game_state/${selectedGameId}/new_player/${playerName}`;
        setEndpoint(endpoint);
    }

    return (
        <div>
            <div>
                <input 
                    type="text" 
                    value={endpoint} 
                    onChange={e => setEndpoint(e.target.value)} 
                    placeholder="/game_state/0"
                />
                <button onClick={sendRequest}>Send</button>
                <button onClick={clearLogs}>Clear</button>
            </div>
            <div>
                <textarea value={responseLog} rows="10" cols="50" readOnly style={{ whiteSpace: 'pre' }}></textarea>
            </div>
            <button onClick={onNewPlayerDebug}>New Player</button>
            <input type="text" value={playerName} onChange={e => setPlayerName(e.target.value)} placeholder={`player_${selectedGameId}`}/>
        </div>
    );
}

export default DebuggingTool;