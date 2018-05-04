#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int main(void) {
  ll k, n, s, p;
  cin >> k >> n >> s >> p;
  ll ns = (n + s - 1) / s;
  cout << (k * ns + p - 1) / p << endl;
}
