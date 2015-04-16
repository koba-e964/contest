object Main extends scala.App {
  import java.{util => ju}
  import scala.annotation._
  import scala.collection._
  import scala.collection.{mutable => mu}
  import scala.collection.JavaConverters._
  import scala.math._

  val sc = new ju.Scanner(System.in)
  val n, k = sc.nextInt
  val t = Array.fill(n)(sc.nextInt)
  val ans = ((3 to n).filter(x => t(x-3) + t(x-2) + t(x-1) < k)) :+ (-1)
  println(ans.head)
  /* code here */
}
