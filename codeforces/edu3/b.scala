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
  val n, m = sc.nextInt
  val nl: Long = n
  val g = Array.fill(n)(sc.nextInt)
  val acc = (1 to m).map(i => g.count(_ == i).asInstanceOf[Long]).map(x => x * (x - 1) / 2).sum
  println(nl * (nl - 1) / 2 - acc)
}