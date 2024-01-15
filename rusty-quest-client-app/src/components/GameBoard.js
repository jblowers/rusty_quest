


// SpaceComponent - A component for individual spaces on the board
const SpaceComponent = ({ space, isActive }) => {
    const style = {
      padding: '10px',
      margin: '5px',
      border: '1px solid black',
      backgroundColor: isActive ? 'lightgreen' : 'white',
    };
  
    return (
      <div style={style}>
        {/* You can add more details about the space here */}
        Space {space.id}
      </div>
    );
  };
  
  // GameBoard - Main component for the game board
  const GameBoard = ({ spaces, activePlayerPosition }) => {
    return (
      <div style={{ display: 'flex', flexWrap: 'wrap' }}>
        {spaces.map((space, index) => (
          <SpaceComponent 
            key={space.id} 
            space={space} 
            isActive={index === activePlayerPosition}
          />
        ))}
      </div>
    );
  };
  
  export default GameBoard;