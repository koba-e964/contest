import java.io.*;
import java.util.*;
/*
	Solve.java
	https://codeiq.jp/ace/joboffer_apli/q446 (川渡り)を解くプログラム
	兵士の数をm人、巨人の数をn体とする。
	方針:それぞれの岸に兵士がx人、巨人がy体、ボートが(こちら側:z=0|向こう側:z=1)にある、と言う状態を点(x+(m+1)*y+z*(m+1)*(n+1))番で表す(計2*(m+1)*(n+1)個)。
	それぞれの状態から別の状態へ遷移できるときに、各状態を表す点の間に矢印を引く。
	最短の解に限らなければ無限個の解が存在する(無駄な移動を繰り返せばよい)ので、最短の解のみをすべて求める。
*/

public class Solve{
	private int m,n;//兵士の数,巨人の数
	private int np;//2(m+1)(n+1),点の数
	private boolean[][] edges;//辺の有無、true:あり、false:なし
	/*
		初期化(m:兵士の数,n:巨人の数,boat:ボートに乗れる兵士・巨人の数)
	*/
	Solve(int m,int n,int boat){
		if(m<n){
			throw new IllegalArgumentException("兵士の方が少ない");
		}
		this.m=m;
		this.n=n;
		this.np=2*(m+1)*(n+1);
		this.edges=new boolean[np][np];
		//false埋め。実は必要なし
		for(boolean[] row:edges){
			Arrays.fill(row,false);
		}
		/*ボートに乗る兵士,巨人の数を(k,l)とすると(k==0またはk>=l)かつk+l<=boat、問題の場合はboat=2なので前半の制限は無いのと同じ*/
		/*ボートがこちら側にある場合*/
		for(int i=0;i<=m;i++){ //こちら側の兵士の数
			for(int j=0;j<=n;j++){ //巨人の数
				if(!isValid(i,j))continue; //遷移前の状態が良いかどうか
				int p1=toPoint(i,j,0); //遷移前の点
				for(int k=0;k<=boat;k++){ //ボートに乗る兵士の数
					for(int l=0;l<=boat-k;l++){ //ボートに乗る巨人の数
						if(k>=1 && k<l)continue; //ボート内で大惨事不可避
						if(!isValid(i-k,j-l))continue; //遷移後の状態が良いかどうか
						int p2=toPoint(i-k,j-l,1);
						edges[p1][p2]=true;
					}
				}
			}
		}
	}
	/*
		各岸における兵士の数の正当性を調べる
		soldier:こちら側の兵士の数
		titan:こちら側の巨人の数
	*/
	boolean isValid(int soldier,int titan){
		return soldier>=0 && titan>=0 && //こちら側で兵士と巨人の数が両方0以上かどうか
			soldier<=m && titan<=n && //向こう側で兵士と巨人の数が両方0以上かどうか
			(soldier>=titan || soldier==0)&& //こちら側で兵士の方が多いまたは0人かどうか
			(m-soldier>=n-titan || m-soldier==0); //向こう側で兵士の方が多いまたは0人かどうか
	}
	/*
		状態を表す点の番号を返す。
		soldier:こちら側の兵士の数
		titan:こちら側の巨人の数
		boatplace:ボートがこちらなら0,向こうなら1
	*/
	private int toPoint(int soldier, int titan, int boatplace){
		assert isValid(soldier,titan);
		return soldier+(m+1)*titan+(m+1)*(n+1)*boatplace;
	}
	/*
		あり得る解をすべて見つける
		見つけるだけで出力はしない
	*/
	public void solve(){
		
	}
	public void print(){
	}
	protected void debugPrint(){
		int v=(m+1)*(n+1);
		for(int x=0;x<v;x++){
			int i=x%(m+1);
			int j=x/(m+1);
			System.out.print(isValid(i,j)?"o ":"x ");
		}
		System.out.println();
		for(int i=0;i<v;i++){
			for(int j=0;j<v;j++){
				System.out.print(edges[i][j+v]?"* ":". ");
			}
			System.out.println();
		}
	}
	/*
		引数argsは無視される。
	*/
	public static void main(String[] args){
		int m=3;//兵士の数
		int n=3;//巨人の数
		int boat=2; // ボートに乗れる人員の数
		assert m>=n;//兵士の方が多くなければ絶対に不可能
		Solve sol=new Solve(m,n,boat);//初期化
		sol.debugPrint();//デバッグプリント
		sol.solve();//解をすべて求める
		sol.print();//解をすべて出力
	}
}