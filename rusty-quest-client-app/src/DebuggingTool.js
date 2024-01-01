import React, { useState } from 'react';

const IP_ENDPOINT = "http://yourserver.com"; // Replace with your server's IP

function DebuggingTool(ipAddress) {
    const [endpoint, setEndpoint] = useState('');
    const [responseLog, setResponseLog] = useState('');

    const sendRequest = () => {
        const fullUrl = ipAddress + endpoint;
        const requestLog = `Request: ${fullUrl}\n`;

        fetch(fullUrl)
            .then(response => response.json())
            .then(data => {
                const formattedResponse = JSON.stringify(data, null, 2);
                setResponseLog(prevLog => prevLog + requestLog + "Response:\n" + formattedResponse + "\n\n");
            })
            .catch(error => {
                setResponseLog(prevLog => prevLog + requestLog + "Error: " + error + "\n\n");
            });
    };

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
            </div>
            <div>
                <textarea value={responseLog} rows="10" cols="50" readOnly style={{ whiteSpace: 'pre' }}></textarea>
            </div>
        </div>
    );
}

export default DebuggingTool;