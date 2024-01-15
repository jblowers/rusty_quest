

const PlayerHand = ({ cards }) => {
    return (
      <div>
        {cards.map((card, index) => (
          <CardComponent key={index} card={card} />
        ))}
      </div>
    );
  };