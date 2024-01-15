import CardComponent from './CardComponent';

const CardCollectionComponent = ({ cards, title}) => {

  const containerStyles = {
    maxHeight: '300px', // Set a fixed height or max-height
    overflowX: 'scroll', // Enable vertical scrolling
    border: '1px solid #ccc', // Optional, for better visibility
    padding: '10px',
    margin: '10px 0',
    maxWidth: '500px',
    display: 'flex',
    flexDirection: 'row'
  };
    return (
      <div>
        <div>
        {title ?
          <h4>{title} Contents</h4>
          :
          <h4>Card Contents</h4>
        }</div> 
        <div style={containerStyles} className="card-collection-container">
          {cards.map((card, index) => (
            <div className="text-container">
              <CardComponent key={index} card={card} />
            </div>
          ))}
        </div>
      </div>
    );
  };

  
export default CardCollectionComponent;