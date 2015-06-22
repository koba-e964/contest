#include <vector>
#include <string>
/*
 * Reference: http://codeforces.com/blog/entry/3107
 * header requirement: vector, string
 */
std::vector<int> gen_z(const std::string &s) {
  int L = 0, R = 0;
  int n = s.length();
  std::vector<int> z(n);
  z[0] = n;
  for (int i = 1; i < n; i++) {
    if (i > R) {
      L = R = i;
      while (R < n && s[R-L] == s[R]) R++;
      z[i] = R-L; R--;
    } else {
      int k = i-L;
      if (z[k] < R-i+1) {
	z[i] = z[k];
      }
      else {
	L = i;
	while (R < n && s[R-L] == s[R]) {
	  R++;
	}
	z[i] = R-L;
	R--;
      }
    }
  }
  return z;
}
#include <iostream>
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 1001000;
const ll mod = 1e9 + 7;
ll pw[N] = {};
int a[N];

int tbl[N] = {};
ll hsh[N];


void init_rolling_hash(const string &p) {
  ll tmp = 1;
  const ll base = 1e9 + 9;
  REP(i, 0, N) {
    pw[i] = tmp;
    tmp *= base;
  }
  tmp = 0;
  hsh[p.length()] = 0;
  for(int i = p.length() - 1; i >= 0; --i) {
    tmp += p[i] - 'a';
    hsh[i] = tmp;
    tmp *= base;
  }
}



int main(void){
  int n, m;
  string p;
  cin >> n >> m >> p;
  init_rolling_hash(p);
  vector<int> z = gen_z(p);
  REP(i, 0, m) {
    cin >> a[i];
    a[i]--;
    tbl[a[i]]++;
    tbl[a[i] + p.length()]--;
  }
  REP(i, 0, N - 1) {
    tbl[i + 1] += tbl[i];
  }
  ll sum = 1;
  REP(i, 0, n) {
    if(tbl[i] == 0) {
      sum *= 26;
      sum %= mod;
    }
  }
  int pl = p.length();
  REP(i, 0, m - 1) {
    int d = a[i + 1] - a[i];
    if (d >= pl) {
      continue;
    }
    // check whether p[d ... |p|] == p[0 ... |p| - d]
    //if (hsh[d] - hsh[0] + hsh[pl - d] * pw[pl - d]) {
    if (z[d] != pl - d) {
      sum = 0;
    }
  }
  cout << sum << endl;
}
