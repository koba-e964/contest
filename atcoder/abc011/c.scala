/*
 * Reference: http://lizan.asia/blog/2012/12/11/scala-competitive/
 */

object Main extends App {
  import java.{util => ju}
  import scala.annotation._
  import scala.collection._
  import scala.collection.{mutable => mu}
  import scala.collection.JavaConverters._
  import scala.math._

  val sc = new ju.Scanner(System.in)
  val n = sc.nextInt
  val ban = Array.fill(3)(sc.nextInt).toSet
  val dp = Array.fill(n + 1)(200)
  dp(0) = 0
  for (i <- 1 to n) {
    for (j <- 1 to (i min 3)) {
      if (!ban(i - j)) {
        dp(i) = dp(i) min (dp(i - j) + 1)
      }
    }
  }
  println (if (!ban(n) && dp(n) <= 100) "YES" else "NO")
}