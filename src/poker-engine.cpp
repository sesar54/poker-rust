#include <algorithm>    // std::adjacent_find
#include <iostream>
#include <utility>      // std::pair
#include <set>          // Using set so having duplicated cards are impossible
#include <vector>       // std::guess_what

using namespace std;

enum hand_type {
    high_card = 0,
    one_pair = 1,
    two_pair = 2,
    three_of_a_kind = 3,
    straight = 4,
    flush = 5,
    full_house = 6,
    four_of_a_kind = 7,
    straight_flush = 8,
};


class PokerEngine {

    unsigned int * hand_score (const set<unsigned> deck ) {

        // First number is type of hand, secound is value of that hand
        static unsigned score[2] = {0,0};

        // Get 3 sorted vectors. One holding suits (diamond, clubs, heart, spades),
        // second holds card numbers (A, 2 , 3.., Queen, King), 
        // and third holds the usual value for card number depending on hand (12, 0, 1.., 10, 11)
        
        const auto deck_sorted = [deck]() {
            vector<unsigned> deck_temp = {deck.begin(), deck.end()};
            sort (deck_temp.begin(), deck_temp.end());
            return deck_temp;
        }();

        // Sorted by number, real card trailing as a pair
        const auto deck_numb = [deck_sorted]() {
            vector<std::pair<unsigned,unsigned>> deck_numb;
            for ( unsigned i=0; i<deck_sorted.size(); i++ ) {
                deck_numb[i] = {deck_sorted[i] % 4, deck_sorted[i]};
            }
            return deck_numb;
        }();


        // Sorted by value, where Ace is the highest value, 2 the lowest, real car trailing as a pair
        const auto deck_value = [deck_numb]() {
            auto deck_value = deck_numb;
            for ( unsigned i=0; i<deck_numb.size(); i++ ) {
                
                // Ace has value 12
                if ( deck_value[i].first == 0 ) { deck_value[i].first = 12; }

                // Everyone else has thier value as its number - 1
                deck_value[i].first = (deck_value[i].first % 4 - 1) ;

            }
            return deck_value;

        }();

        // Sorted by color, real card trailing as a pair
        const auto deck_color = [deck_sorted]() {
            vector<std::pair<unsigned,unsigned>> deck_color;
            for ( unsigned i=0; i<deck_sorted.size(); i++ ) {
                deck_color[i] = {deck_sorted[i] % 13, deck_sorted[i]};
            }
            return deck_color;
        }();


        // Check for High Card, 1, 2 pair, Three of a kind, Full house and Four of a kind
        {
            // Count the number of cards by value, not by color. Ace being highest valuie.
            const auto deck_counted = [deck_value]() {

                unsigned i=0;

                vector <std::pair<unsigned,unsigned>> deck_counted {0,deck_value.begin.secound};

                unsigned last_card = deck_value.begin.firsts;

                for ( auto card : deck_value ) {

                    if (card.first == last_card) {

                        deck_counted[i].first++;

                    } else {

                        last_card = card.first;

                        deck_counted[++i] = {1,card.second};

                    }


                };

                return deck_counted;

            }();


            // Check 
            

        }

        /*
        // High Card
        for ( auto card: deck_value ) {
            
            if ( score[1] < card ) { score[1] = card; }

        }

        // NOTE: Sorting does put same card next to each other but not PAIRS!
        // Induvidual pair checking required for each type of pair
        
        // Pair
        for ( unsigned pos[] = {0,1}; pos[1] < deck.size(); pos[0]++, pos[1]++ ) {

            auto* card0 = &deck_value[pos[0]];
            auto* card1 = &deck_value[pos[1]];

            if (*card0 == *card1) {
                score[0] = 1;
                score[1] = *card0;
            }

        }


        // Two Pair
            for ( unsigned pos[] = {0,1,2,3}; pos[5] < deck.size(); pos[0]++, pos[1]++, pos[2]++, pos[3]++ ) {

                auto* card0 = &deck_value[pos[0]];
                auto* card1 = &deck_value[pos[1]];
                auto* card2 = &deck_value[pos[2]];
                auto* card3 = &deck_value[pos[3]];

                if (*card0 == *card1 && *card2 == *card3) {
                    score[0] = two_pair;
                    score[1] = *card0;
                }

            }


        // Three of a kind
        unsigned pos[] {0,0,0};
        for (auto card0: deck) { for (auto card1: deck) { for (auto card2: deck) {
            
            vector<unsigned> hand = {card0,card1,card2};

            // Check so that all cards are unique
            if ( adjacent_find(hand.begin(), hand.end()) != hand.end() ) {
                continue;
            };

            // Removing color from card
            card0 = card0 % 4;
            card1 = card1 % 4;
            card2 = card2 % 4;

            if (card0 = card1 && card1 == card2) {
                score[0] = three_of_a_kind;
                score[1] = card0;
            }
        
        }}}

        */

        // Straight
        unsigned pos[] {0,0,0,0,0};
        for (unsigned i=0; i< (deck.size()-5); i++ ) {

            if ( card0 == (card1+1) && card1 == (card2+1) && card2 == (card3+1) && card3 == (card4+1) ) {
                score[0] = straight;
                score[1] = card0;
            };
    
        }


        // Flush
        for (auto iter0 = deck.begin(); iter0 != deck.end(); iter0++) { for (auto iter1 = (++iter0--); iter1 !=deck.end(); iter1 ) { for (auto iter2 = (++iter1--); iter2 !=deck.end(); iter2++ ) { for (auto iter3 = (++iter2--); iter3 !=deck.end(); iter3++ ) { for (auto iter4 = (++iter3--); iter4 !=deck.end(); iter4++ ) {

            unsigned card0 = *iter0 % 4;
            unsigned card1 = *iter0 % 4;
            unsigned card2 = *iter0 % 4;
            unsigned card3 = *iter0 % 4;
            unsigned card4 = *iter0 % 4;

        }


        return score;

    };
    

    struct player {

    };


    struct deck {

    };

};