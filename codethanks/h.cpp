#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

const int R = 251;

int r,c;

string s[R];

int check(int p, int q, int x, int y) { // [x,p-x], [y, q-y]
	REP(i,x,p-x+1) {
		REP(j, y, q-y+1) {
			if ( s[i][j] != s[p-i][q-j]) {
				return 0;
			}
		}
	}
	return 1;
}


const int U= 256;

/**
 * holds the array (1.. size)
 *
 */
class BIT {
	int ary[257][257];
public:
	BIT() {
		
		for(int i=0; i<=U; i++) {
			ary[i] = 0;
		}
	}
	~BIT(void) {
	}
	/**
	 * gets the sum in [1 .. ind]
	 * @param ind
	 * @return sum
	 */
	int accum(int ind, int ind2) {
		T sum=0;
		while(ind>0) {
			sum += ary[ind][;
			ind &= ind - 1;
		}
		return sum;
	}
	/**
	 * performs data[ind] += val;
	 */
	void add(int ind, int val) {
		while(ind <= n) {
			ary[ind] += val;
			ind += ind & (-ind);
		}
	}
	T size() {
		return ary[n];
	}
};
 



const int T = 16;

int bit[T*T][T*T];


int sq[T][T];

int prep[T][T];


int dp[U][U];

void clear(void) {
	REP(i,0, T*T) {
		REP(j,0,T*T) {
			bit[i][j] = 0;
		}
	}
	REP(i,0, T) {
		REP(j,0,T) {
			sq[i][j] = 0;
		}
	}
	REP(i,0,T) {
		REP(j,0,T) {
			dp[i][j] = -1;
		}
	}
}

void bits(int i, int j, int b) {
	sq[i/T][j/T] += b - bit[i][j];
	bit[i][j] = b;
}

void check(int p, int q, int x, int y) {
	int txa = (x + T - 1) / T * T;
	int txb = (p - x) / T * T;
	int tya = (y + T - 1) / T * T;
	int tyb = (q - y) / T * T;
	if (x == p - x || y == q - y) {
		return 1;
	}
	if ()
}


int main(void){
	cin >> r >> c;
	REP(i,0,r) {
		cin >> s [i];
	}
	if(c *r >= 500) exit(1);
	int cnt = 0;
	REP(p, 0, 2 * r) {
		REP(q, 0, 2 * c) {
			int xa = max(p-r, 0);
			int xb = p - xa - 1;
			int ya = max(q-c, 0);
			int yb = q - ya - 1;
			REP(i,xa, xb+1) {
				REP(j, ya, yb+1) {
					bits(i,j, s[i][j] != s[p-i][q-j] ? 0 : 1);
				}
			}
			
		}
	}
	cout << cnt << endl;
}
