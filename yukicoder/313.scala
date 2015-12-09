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
  val dig = sc.next
  val ans = Array(20104, 20063, 19892, 20011, 19874, 20199, 19898, 20163, 19956, 19841)
  val diff = Array.tabulate(10){x => dig.count(_ == x + '0') - ans(x)}
  println(diff.indexOf(1) + " " + diff.indexOf(-1))
}