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
typedef pair<int, int> PI;
const double EPS=1e-9;


int parse(const string &s) {
  int q = 0;
  switch(s[0]) {
  case 'D':
    q = 0;
    break;
  case 'C':
    q = 1;
    break;
  case 'H':
    q = 2;
    break;
  case 'S':
    q = 3;
    break;
  }
  q *= 13;
  if (s[1] == 'A') {
    q += 1; 
  } else if (s[1] == 'T') {
    q += 10;
  } else if (s[1] == 'J') {
    q += 11;
  } else if (s[1] == 'Q') {
    q += 12;
  } else if (s[1] == 'K') {
    q += 13;
  } else {
    q += s[1] - '0';
  }
  return q;
}

int main(void){
  int n;
  cin >> n;
  vector<pair<int, string> > pool;
  REP(i, 0, n) {
    string s;
    cin >> s;
    pool.push_back(pair<int, string>(parse(s), s));
  }
  sort(pool.begin(), pool.end());
  REP(i, 0, n) {
    cout << pool[i].second << (i == n - 1 ? "\n" : " ");
  }
}
