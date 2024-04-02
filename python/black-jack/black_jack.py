"""Functions to help play and score a game of blackjack.

How to play blackjack:    https://bicyclecards.com/how-to-play/blackjack/
"Standard" playing cards: https://en.wikipedia.org/wiki/Standard_52-card_deck
"""

ROYAL = {'J', 'Q', 'K'}


def value_of_card(card):
    """Determine the scoring value of a card.

    :param card: str - given card.
    :return: int - value of a given card.  See below for values.

    1.  'J', 'Q', or 'K' (otherwise known as "face cards") = 10
    2.  'A' (ace card) = 1
    3.  '2' - '10' = numerical value.
    """
    value = 0
    if card in ROYAL:
        value = 10
    elif card == 'A':
        value = 1
    else:
        value = int(card)
    return value




def higher_card(card_one, card_two):
    """Determine which card has a higher value in the hand.

    :param card_one, card_two: str - cards dealt in hand.  See below for values.
    :return: str or tuple - resulting Tuple contains both cards if they are of equal value.

    1.  'J', 'Q', or 'K' (otherwise known as "face cards") = 10
    2.  'A' (ace card) = 1
    3.  '2' - '10' = numerical value.
    """
    v1 = value_of_card(card_one)
    v2 = value_of_card(card_two)
    
    if v1 == v2:
        return card_one, card_two
    
    if v1 > v2:
        winner = card_one
    else:
        winner = card_two
    return winner



def value_of_ace(card_one, card_two):
    """Calculate the most advantageous value for the ace card.

    :param card_one, card_two: str - card dealt. See below for values.
    :return: int - either 1 or 11 value of the upcoming ace card.

    1.  'J', 'Q', or 'K' (otherwise known as "face cards") = 10
    2.  'A' (ace card) = 11 (if already in hand)
    3.  '2' - '10' = numerical value.
    """
    has_ace = 'A' in {card_one, card_two}
    val = value_of_card(card_one) + value_of_card(card_two)
    if has_ace:
        val = val + 10

    if val + 11 <= 21:
        ace = 11
    else:
        ace = 1
    return ace


def is_blackjack(card_one, card_two):
    """Determine if the hand is a 'natural' or 'blackjack'.

    :param card_one, card_two: str - card dealt. See below for values.
    :return: bool - is the hand is a blackjack (two cards worth 21).

    1.  'J', 'Q', or 'K' (otherwise known as "face cards") = 10
    2.  'A' (ace card) = 11 (if already in hand)
    3.  '2' - '10' = numerical value.
    """
    vals = {value_of_card(card_one), value_of_card(card_two)}

    if 'A' in {card_one, card_two} and 10 in vals:
        natural = True
    else:
        natural = False
    return natural



def can_split_pairs(card_one, card_two):
    """Determine if a player can split their hand into two hands.

    :param card_one, card_two: str - cards dealt.
    :return: bool - can the hand be split into two pairs? (i.e. cards are of the same value).
    """
    v1 = value_of_card(card_one)
    v2 = value_of_card(card_two)
    if v1 == v2:
        pair = True
    else:
        pair = False
    return pair


def can_double_down(card_one, card_two):
    """Determine if a blackjack player can place a double down bet.

    :param card_one, card_two: str - first and second cards in hand.
    :return: bool - can the hand can be doubled down? (i.e. totals 9, 10 or 11 points).
    """

    val = value_of_card(card_one) + value_of_card(card_two)

    if val in {9, 10, 11}:
        trouble = True
    else:
        trouble = False
    return trouble
