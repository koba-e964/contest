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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

ll tbl[] = {
  1,
  491101308,
  723816384,
  27368307,
  199888908,
  927880474,
  661224977,
  970055531,
  195888993,
  547665832,
  933245637,
  368925948,
  136026497,
  135498044,
  419363534,
  668123525,
  30977140,
  309058615,
  189239124,
  940567523,
  429277690,
  358655417,
  780072518,
  275105629,
  99199382,
  733333339,
  608823837,
  141827977,
  637939935,
  848924691,
  724464507,
  326159309,
  903466878,
  769795511,
  606241871,
  957939114,
  852304035,
  375297772,
  624148346,
  624500515,
  203191898,
  629786193,
  814362881,
  116667533,
  627655552,
  586445753,
  193781724,
  83868974,
  965785236,
  377329025,
  698611116,
};

void precompute() {
  ll prod = 1;
  REP(i, 1, mod) {
    prod = prod * i % mod;
    if (i % 20000000 == 0) {
      cout << prod << "," << endl;
    }
  }
}


int main(void){
  ll n;
  cin >> n;
  if (n >= mod) {
    cout << 0 << endl;
    return 0;
  }
  const int B = 20000000;
  int t = n / B;
  ll prod = tbl[t];
  for (int i = t * B + 1; i <= n; ++i) {
    prod = prod * i % mod;
  }
  cout << prod << endl;
}
