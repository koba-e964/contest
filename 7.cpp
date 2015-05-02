#include <vector>

/* Required headers: vector */
class Sieve {
private:
  std::vector<int> a;
public:
  Sieve(int n) : a(n + 1) {
    for (int i = 0; i <= n; ++i) {
      a[i] = 1;
    }
    a[0] = a[1] = 0;
    int c = 2;
    while (c <= n) {
      if (a[c] == 0) {
	c++;
	continue;
      }
      for(int i = 2 * c; i <= n; i += c) {
	a[i] = 0;
      }
      c++;
    }
  }
  bool operator[] (int x) const {
    return a[x] != 0;
  }
};


#include <iostream>


#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 10010;
int dp[N];

Sieve sie(0);

int rec(int n) {
  if (n <= 1) {
    return 1;
  }
  if (dp[n] >= 0) {
    return dp[n];
  }
  REP(i, 2, n) {
    if (sie[i]) {
      if (rec(n - i) == 0) {
	dp[n] = 1;
	return 1;
      }
    }
  }
  dp[n] = 0;
  return 0;
}

int main(void){
  int n;
  cin >> n;
  REP(i, 0, N) dp[i] = -1;
  sie = Sieve(n);
  cout << (rec(n) ? "Win" : "Lose") << endl;
}
