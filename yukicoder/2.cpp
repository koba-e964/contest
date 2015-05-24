#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>


using namespace std;
class Power {
	typedef long long i64;
public:
	/* a^b with no modulo operations */
	static long long power(long long a, long long b) {
		i64 s = 1;
		i64 c = a;
		while (b > 0) {
			if (b % 2) {
				s *= c;
			}
			c *= c;
			b /= 2;
		}
		return s;
	}
	/* return (a,b) s.t, n = a^b and b is maximal. O(64) */
	static pair<long long, int> toPower(long long n) {
		for (int i = 64; i >= 2; i--) {
			i64 app = pow(n, 1.0/i);
			for (int d = -4; d <= 4; d++) {
				i64 x = app + d;
				if (x <= 0) continue;
				if (power(x, i) == n) {
					return pair<i64, int>(x, i);
				}
			}
		}
		return pair<i64, int>(n, 1);
	}
	/* factorize n and returns prime factors and their exponents.  O(sqrt(n)) */
	static vector<pair<long long, int> > factorize(long long n) {
		vector<pair<i64, int> > res;
		i64 p = 2;
		int c = 0;
		while (n >= 1) {
			if (c == 0 && n < p * p) {
			  if(n != 1) {
			    res.push_back(pair<i64, int>(n,1));
			  }
			  break;
			}
			if (n % p != 0) {
				if (c > 0) {
					res.push_back(pair<i64, int>(p,c));
				}
				p++;
				c = 0;
				continue;
			}
			n /= p;
			c++;
		}
		return res;
	}
};

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;



int main(void){
  ll n;
  cin >> n;
  vector<pair<ll, int> > fac = Power::factorize(n);
  int k = 0;
  REP(i, 0, fac.size()) {
    k ^= fac[i].second;
  }
  cout << (k ? "Alice" : "Bob") << endl;
}
