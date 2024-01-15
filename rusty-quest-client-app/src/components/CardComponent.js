


const CardComponent = ({ card }) => {

    const valueColor = card.typ === 'Bad' ? 'red' : 'black';

    return (
      <div>
        <p style={{ color: valueColor }}>Value: {card.value}</p>
        {/* Add more card details */}
      </div>
    );
  };


  
export default CardComponent;