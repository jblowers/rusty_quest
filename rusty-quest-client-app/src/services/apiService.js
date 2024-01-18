// import { useIpAddress } from '../contexts/IpAddressContext';

const fetchData = async (ipAddress, endpoint, options = {}) => {
    // const { ipAddress } = useIpAddress();
    try {
        console.log(`Attempting to fetch from: ${ipAddress}${endpoint}`)
        const response = await fetch(`${ipAddress}${endpoint}`, options);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data = await response.json();


        return data; // Return the original data if the nested structure is not present
    } catch (error) {
        console.error('Fetch error:', error);
        throw error;
    }
};

const postData = async (ipAddress, endpoint, data, options={}) => {
    const requestOptions = {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data),
        ...options
    };
    try {
        const response = await fetch(`${ipAddress}${endpoint}`, requestOptions);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return await response.json();
    } catch (error) {
        console.error('Post error:', error);
        throw error;
    }
};

export const fetchGameState = async (ipAddress,gameId) => {
    const data = await fetchData(ipAddress,`/game_state/${gameId}`);
    // Check if data has a nested gameState property and return it if present
    if (data && data.gameState) {
        console.log(`found gameState.`);
        return data.gameState;
    }
    return data;
};

export const fetchActionInfoList = async (ipAddress) => {
    return fetchData(ipAddress, '/actionlistinfo');
};

export const fetchGameList = async (ipAddress) => {
    return await fetchData(ipAddress, '/game_list');
}

export const sendPlayerAction = async (ipAddress, gameId, playerId, actionData) => {
    const data = await postData(ipAddress,`/game_state/${gameId}/player/${playerId}/action`,actionData);
    return data;
}


// Add more API functions as needed
