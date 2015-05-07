#include <iostream>


using namespace std;
typedef long long ll;
const int M = 1e6;
const int mod = 1e9+7;

int main(){
  ll sum = 1;
  cout << "int fact_tbl[1001] = {" << endl;
  for (int i = 0; i < 1001; i++) {
    cout << sum << "," << endl;
    if (i == 1000) break;
    for (int j = 0; j < M; ++j) {
      sum *= i * M + j + 1;
      sum %= mod;
    }
  }
  cout << "};" << endl;
}
