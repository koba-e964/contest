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

PI read(const string &s) {
  stringstream is(s);
  int a, b;
  is >> a;
  is.ignore();
  is >> b;
  a = (a / 100) * 60 + a % 100;
  b = (b / 100) * 60 + b % 100;
  return PI(a, b);
}

int main(void){
  int n;
  cin >> n;
  vector<PI> dat(n);
  REP(i, 0, n) {
    string s;
    cin >> s;
    dat[i] = read(s);
  }
  vector<bool> raining(288);
  REP(i, 0, n) {
    int start = dat[i].first / 5;
    int end = (dat[i].second + 4) / 5;
    REP(j, start, end) {
      raining[j] = true;
    }
  }
  bool cur = false;
  VI event;
  REP(i, 0, 288) {
    if (raining[i] != cur) {
      event.push_back(i);
      cur = raining[i];
    }
  }
  if (raining[287]) {
    event.push_back(288);
  }
  assert (event.size() % 2 == 0);
  REP(i, 0, event.size() / 2) {
    int a = event[2 * i];
    int b = event[2 * i + 1];
    a *= 5;
    b *= 5;
    a = (a / 60) * 100 + a % 60;
    b = (b / 60) * 100 + b % 60;
    printf("%04d-%04d\n", a, b);
  }
}
