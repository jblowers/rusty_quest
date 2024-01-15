// import { useIpAddress } from '../contexts/IpAddressContext';

const fetchData = async (ipAddress, endpoint, options = {}) => {
    // const { ipAddress } = useIpAddress();
    const response = await fetch(`${ipAddress}${endpoint}`, options);
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }
    return await response.json();
};

export const fetchGameState = async (ipAddress,gameId) => {
    return await fetchData(ipAddress,`/game_state/${gameId}`);
};

export const fetchActionInfoList = async (ipAddress) => {
    return fetchData(ipAddress, '/actionlistinfo');
};

// Add more API functions as needed
