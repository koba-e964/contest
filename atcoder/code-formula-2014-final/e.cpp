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

const int N = 24;
const int B = 1 << 23;

string s;

int fib[N];

void fib_init(void) {
  int x = 0;
  int y = 1;
  REP(i,0,N) {
    fib[i] = x;
    int r = x + y;
    x = y;
    y = r;
  }
}

int dp[N][B];

int mutual(int n, const string &s, const string &t) {
  // cerr << "call(mutual):" << n << " " << s << " " << t << endl;
  if (n <= 2) {
    return 0;
  }
  if (s.substr(0, t.length()) == t) {
    int sub = mutual(n - 1, t, s.substr(t.length()));
    if (sub >= 0) {
      return 2 * sub;
    }
  }
  if (s.substr(s.length() - t.length()) == t) {
    int sub = mutual(n - 1, t, s.substr(0, s.length() - t.length()));
    if (sub >= 0) {
      return 2 * sub + 1;
    }
  }
  return -1;
}

// if not found, return -1.
int rec(int k) {
  if(k == 1) {
    return s.substr(0, fib[k]) == "b" ? 0 : -1;
  }
  if(k == 2) {
    return s.substr(0, fib[k]) == "a" ? 0 : -1;
  }
  // [k-3] [k-2] [k-2] ?
  int f3 = fib[k-3];
  int f2 = fib[k-2];
  if(s.substr(f3,f2) == s.substr(f3 + f2, f2)) {
    int sub = mutual(k - 2, s.substr(f3, f2), s.substr(0, f3));
    if(sub >= 0) {
      return sub * 4 + 2;
    }
  }
  // [k-2] [k-3] [k-2] ?
  if(s.substr(0, f2) == s.substr(f3 + f2, f2)) {
    int sub = mutual(k - 2, s.substr(0, f2), s.substr(f2, f3));
    if(sub >= 0)
      return sub * 4;
  }
  // [k - 2] [k - 2] [k - 3] ?
  if(s.substr(0, f2) == s.substr(f2, f2)) {
    int sub = mutual(k - 2, s.substr(0, f2), s.substr(2 * f2, f3));
    if(sub >= 0)
      return sub * 4 + 1;
  }
  return -1;
}

void enumerate_few(void) {
  vector<vector<string> > dp(10);
  dp[0] = vector<string>(1, "b");
  dp[1] = vector<string>(1, "a");
  REP(i, 2, 6) {
    dp[i] = vector<string>(1 << (i - 1), "");
    REP(k, 0, 1 << (i - 1)) {
      if (k % 2 == 0) {
	dp[i][k] = dp[i - 1][k / 2] + dp[i - 2][k / 4];
      } else {
	dp[i][k] = dp[i - 2][k / 4] + dp[i - 1][k / 2];
      }
    }
    REP(k, 0, 1 << (i - 1)) {
      cerr << "dp[" << i << "][" << k << "]=" << dp[i][k] << endl;
    }
  }
  
}

int main(void){
  cin>>s;
  fib_init();
  //enumerate_few();
  if(s == "a") {
    cout << "2 0" << endl;
    return 0;
  }
  if(s == "b") {
    cout << "1 0" << endl;
    return 0;
  }
  if (s == "ab") {
    cout << "3 0" << endl;
    return 0;
  }
  if (s == "ba") {
    cout << "3 1" << endl;
    return 0;
  }
  int n = -1;
  REP(i,0,N) {
    if((int) s.length() == fib[i]) {
      n=i;
      break;
    }
  }
  cout << n << " " << rec(n) << endl;
}
