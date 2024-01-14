import CardComponent from './CardComponent';

const CardCollectionComponent = ({ cards, title}) => {

  const containerStyles = {
    maxHeight: '300px', // Set a fixed height or max-height
    overflowY: 'scroll', // Enable vertical scrolling
    border: '1px solid #ccc', // Optional, for better visibility
    padding: '10px',
    margin: '10px 0'
  };
    return (
      <div style={containerStyles}>
        {title ?
          <h4>{title} Contents</h4>
          :
          <h4>Card Contents</h4>
        }
        {cards.map((card, index) => (
          <CardComponent key={index} card={card} />
        ))}
      </div>
    );
  };

  
export default CardCollectionComponent;