#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const ll mod = 573;

ll invt[600] = {};

int tot = 0;
int ap[26];

int solve(void) {
  int f3 = 0;
  int f191 = 0;
  ll acc = 1;
  REP(i, 1, tot + 1) {
    int v = i;
    while (v % 3 == 0) {
      v /= 3;
      f3++;
    }
    while (v % 191 == 0) {
      v /= 191;
      f191++;
    }
    acc = acc * v % mod;
  }
  REP(i, 0, 26) {
    REP(j, 1, ap[i] + 1) {
      int v = j;
      while (v % 3 == 0) {
	v /= 3;
	f3--;
      }
      while (v % 191 == 0) {
	v /= 191;
	f191--;
      }
      acc = acc * invt[v % mod] % mod;
    }
  }
  REP(i, 0, f3) {
    acc = acc * 3 % mod;
  }
  REP(i, 0, f191) {
    acc = acc * 191 % mod;
  }
  return (acc + mod - 1) % mod;
}

int main(void){
  REP(i, 1, mod) {
    if (__gcd(i, (int)mod) != 1) {
      continue;
    }
    REP(j, 1, mod) {
      if (i * j % mod == 1) {
	invt[i] = j;
	break;
      }
    }
  }
  string s;
  cin >> s;
  REP(i, 0, s.length()) {
    ap[s[i] - 'A']++;
    tot++;
  }
  cout << solve() << endl;
}
