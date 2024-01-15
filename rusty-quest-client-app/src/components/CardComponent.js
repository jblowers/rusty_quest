


const CardComponent = ({ card }) => {

    const valueColor = card.typ === 'Bad' ? 'red' : 'black';

    return (
      <div className='card-container'>
        <div className='card-text-container'>
            <p style={{ color: valueColor }}>{card.value}</p>
        </div>
        {/* Add more card details */}
      </div>
    );
  };


  
export default CardComponent;