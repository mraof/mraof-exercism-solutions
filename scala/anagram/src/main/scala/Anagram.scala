object Anagram {
  def findAnagrams(word: String, list: List[String]): List[String] = {
    val lowerWord = word.toLowerCase
    val sorted = lowerWord.sorted;
    list.filter(s => {
      val lower = s.toLowerCase;
      lower != lowerWord && sorted.equals(lower.sorted)
    })
  }
}
