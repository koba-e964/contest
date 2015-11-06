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



ll gcd(ll a, ll b) {
    while (b != 0) {
        ll r = a % b;
        a = b;
        b = r;
    }
    return a;
}

int main(void){
  ll n, m;
  cin >> n >> m;
  ll g = gcd(n, m);
  n /= g;
  m /= g;
  if (m == 1) {
    while (n % 10 == 0) {
      n /= 10;
    }
    cout << (n % 10) << endl;
    return 0;
  }
  while (m % 10 == 0) {
    m /= 10;
  }
  if (m % 2 == 0) {
    int c = 0;
    while (m % 2 == 0) {
      c++;
      m /= 2;
    }
    if (m != 1) {
      cout << -1 << endl;
      return 0;
    }
    cout << 5 << endl;
    return 0;
  }
  int c = 0;
  while (m % 5 == 0) {
    c++;
    m /= 5;
  }
  if (m != 1) {
    cout << -1 << endl;
    return 0;
  }
  int ar[4] = {6, 2, 4, 8};
    cout << ((n % 10) * (c == 0 ? 1 : ar[c % 4])) % 10 << endl;

}
