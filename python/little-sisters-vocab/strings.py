"""
Various functions to manipulate words and sentences.
"""

import re

def add_prefix_un(word):
    """

    :param word: str of a root word
    :return:  str of root word with un prefix

    This function takes `word` as a parameter and
    returns a new word with an 'un' prefix.
    """

    return f"un{word}"






def make_word_groups(vocab_words):
    """

    :param vocab_words: list of vocabulary words with a prefix.
    :return: str of prefix followed by vocabulary words with
             prefix applied, separated by ' :: '.

    This function takes a `vocab_words` list and returns a string
    with the prefix  and the words with prefix applied, separated
     by ' :: '.
    """

    return vocab_words[0] + " :: " + " :: ".join(vocab_words[0] + word for word in vocab_words[1:])


def remove_suffix_ness(word):
    """

    :param word: str of word to remove suffix from.
    :return: str of word with suffix removed & spelling adjusted.

    This function takes in a word and returns the base word with `ness` removed.
    """

    # Remove "ness" suffix
    word_without_ness = word.split("ness")[0]

    # Adjust spelling if word ended in "iness" to end in "y"
    if word_without_ness[-1] == "i":
        word_without_ness = word_without_ness[:-1] + "y"
    return word_without_ness


def adjective_to_verb(sentence, index):
    """

    :param sentence: str that uses the word in sentence
    :param index:  index of the word to remove and transform
    :return:  str word that changes the extracted adjective to a verb.

    A function takes a `sentence` using the
    vocabulary word, and the `index` of the word once that sentence
    is split apart.  The function should return the extracted
    adjective as a verb.
    """

    sentence_list = re.split(r'\W',sentence)

    # Remove empty delimiters
    sentence_list = [word for word in sentence_list if word != '']

    return sentence_list[index] + 'en'
