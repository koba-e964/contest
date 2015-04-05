#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;


int main(void){
	int n,m;
	cin>>n>>m;
	srand(13);
	while(1){
		int k=rand()%n;
		k++;
		if(k!=m){
			cout<<k<<endl;
			return 0;
		}
	}
	return -1;
}
