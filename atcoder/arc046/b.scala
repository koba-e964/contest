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
  val n, a, b = sc.nextInt
  val win = if (a == b) n % (a + 1) != 0 else n <= a || a > b
  println(if (win) "Takahashi" else "Aoki")
}