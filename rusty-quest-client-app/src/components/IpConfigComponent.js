import React, { useEffect, useState, useCallback  } from 'react';

const IpAddressConfiguration = ({ ipAddress, applyIpAddress }) => {
    const [currIp, setCurrIp] = useState(ipAddress);
    const [tempIp, setTempIp] = useState('');

    const handleIpChange = (event) => {
        setTempIp(event.target.value);
      };

    function loadSelectedIp(event) {
        setTempIp(event.target.value);
    }
    
// function CardList({ cards, shuffleClickHandler }) {

    return (
        <div className='server-ip-config-container'>  
            <div className='text-container'>
                <h2>Server Configuration</h2>
                <label htmlFor="currentIpInput">Current IP </label>
                <input id="currentIpInput" type="text" value={currIp} readOnly />
                <br />
                <hr />
                <label htmlFor="ipToSetInput">IP to set </label>
                <input id="ipToSetInput" type="text" value={tempIp} onChange={handleIpChange} />
                <button onClick={applyIpAddress} value={tempIp}>Apply IP Address</button>
                <br />
                <label htmlFor="autoLoadSelect">Auto load </label>
                <select id="autoLoadSelect" value={tempIp} onChange={loadSelectedIp}>
                    <option value="http://localhost:3030">http://localhost:3030</option>
                    <option value="http://192.168.0.134:3030">http://192.168.0.134:3030</option>
                    <option value="http://localhost:3030">http://localhost:3030</option>
                </select>
                <hr />
            </div>
        </div>
    );
};

export default IpAddressConfiguration;
