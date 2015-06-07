#include <iostream>
#include <string>
#include <vector>
#include <cstdio>
#include <cmath>
#include <sstream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

const ll M = 1e10;

int main(void){
  int n;
  cin >> n;
  ll s0 = 0, s1 = 0;
  REP(i, 0, n) {
    string line;
    ll q;
    cin >> line;
    stringstream ls;
    ls << line;
    ls >> q;
    char o = ls.get();
    ll r = 0;
    if (o == '.') {
      string s;
      ls >> s;
      while (s.length() < 10) {
	s += '0';
      }
      stringstream ss;
      ss << s;
      ss >> r;
    }
    if (line[0] == '-') {
      r = -r;
    }
    s0 += q;
    s1 += r;
  }
  ll q = s1 / M;
  s0 += q;
  s1 -= q * M;
  int sgn = s0 < 0;
  if (sgn) {
    s0 = -s0;
    s1 = -s1;
  }
  if (s0 > 0 && s1 < 0) {
    s0--;
    s1 += M;
  }
  s1 = abs(s1);
  if (sgn) {
    printf("-");
  }
  printf("%lld.%010lld\n", s0, s1);
}
