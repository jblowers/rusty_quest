import React, { createContext, useState, useContext } from 'react';

export const IpAddressContext = createContext();

const DEFAULT_URL = 'http://192.168.0.134:3030';

export const IpAddressProvider = ({ children }) => {
    const [ipAddress, setIpAddress] = useState(DEFAULT_URL); // Set your default IP address

    return (
        <IpAddressContext.Provider value={{ ipAddress, setIpAddress }}>
            {children}
        </IpAddressContext.Provider>
    );
};

export const useIpAddress = () => useContext(IpAddressContext);
