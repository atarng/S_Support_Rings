Include("Common")

function addRings()
	if (CheckSupportRank("PID_リュール", "PID_ヴァンドレ", 3)) then
		if ( TryVariable("G_所持_RNID_VanderBond_S_c") == false ) then
			Log("addRings] Check Alear and Vander Support: Success")
			VariableEntry("G_所持_RNID_VanderBond_S_c", 1)
			AddBondRing("RNID_VanderBond_S", 1)
		end
	else
		Log("addRings] Check Alear and Vander Support: Failure")
	end
	
	if (CheckSupportRank("PID_リュール", "PID_クラン", 3)) then
		if ( TryVariable("G_所持_RNID_ClanneBond_S_c") == false ) then
			Log("addRings] Check Alear and Clanne Support: Success")
			VariableEntry("G_所持_RNID_ClanneBond_S_c", 1)
			AddBondRing("RNID_ClanneBond_S", 1)
		end
	else
		Log("addRings] Check Alear and Clanne Support: Failure")
	end

	if (CheckSupportRank("PID_リュール", "PID_フラン", 3)) then
		if ( TryVariable("G_所持_RNID_FrammeBond_S_c") == false ) then
			Log("addRings] Check Alear and Framme Support: Success")
			VariableEntry("G_所持_RNID_FrammeBond_S_c", 1)
			AddBondRing("RNID_FrammeBond_S", 1)
		end
	else
		Log("addRings] Not enough Support Framme.")
	end
	
	if (CheckSupportRank("PID_リュール", "PID_アルフレッド", 3)) then
		if ( TryVariable("G_所持_RNID_AlfredBond_S_c") == false ) then
			Log("addRings] Check Alear and Alfred Support: Success")
			VariableEntry("G_所持_RNID_AlfredBond_S_c", 1)
			AddBondRing("RNID_AlfredBond_S", 1)
		end
	else
		Log("addRings] Check Alear and Alfred Support: Failure")
	end

	if ( TryVariable("G_所持_RNID_EtieBond_S_c") == false ) then
		if (CheckSupportRank("PID_リュール", "PID_エーティエ", 3)) then
			Log("addRings] Check Alear and Etie Support: Success")
			VariableEntry("G_所持_RNID_EtieBond_S_c", 1)
			AddBondRing("RNID_EtieBond_S", 1)
		end
	else
		Log("addRings] Check Alear and Etie Support: Failure")
	end

	if (CheckSupportRank("PID_リュール", "PID_ブシュロン", 3)) then
		if ( TryVariable("G_所持_RNID_BoucheronBond_S_c") == false ) then
			VariableEntry("G_所持_RNID_BoucheronBond_S_c", 1)
			AddBondRing("RNID_BoucheronBond_S", 1)
		end
	end

	if (CheckSupportRank("PID_リュール", "PID_セリーヌ", 3)) then
		if ( TryVariable("G_所持_RNID_CelineBond_S_c") == false ) then
			VariableEntry("G_所持_RNID_CelineBond_S_c", 1)
			AddBondRing("RNID_CelineBond_S", 1)
		end
	end

	if (CheckSupportRank("PID_リュール", "PID_クロエ", 3)) then
		if ( TryVariable("G_所持_RNID_ChloeBond_S_c") == false ) then
			VariableEntry("G_所持_RNID_ChloeBond_S_c", 1)
			AddBondRing("RNID_ChloeBond_S", 1)
		end
	end

	if (CheckSupportRank("PID_リュール", "PID_ルイ", 3)) then
		if ( TryVariable("G_所持_RNID_LouisBond_S_c") == false ) then
			Log("addRings] Check Alear and Louis Support: Success")
			VariableEntry("G_所持_RNID_LouisBond_S_c", 1)
			AddBondRing("RNID_LouisBond_S", 1)
		end
	else
		Log("addRings] Check Alear and Louis Support not reached.")
	end

	if (CheckSupportRank("PID_リュール", "PID_ユナカ", 3)) then
		if ( TryVariable("G_所持_RNID_YunakaBond_S_c") == false ) then
			VariableEntry("G_所持_RNID_YunakaBond_S_c", 1)
			AddBondRing("RNID_YunakaBond_S", 1)
		end
	end

	if ( TryVariable("G_所持_RNID_AlcrystBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_スタルーク", 3)) then
		VariableEntry("G_所持_RNID_AlcrystBond_S_c", 1)
		AddBondRing("RNID_AlcrystBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_CitrinneBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_シトリニカ", 3)) then
		VariableEntry("G_所持_RNID_CitrinneBond_S_c", 1)
		AddBondRing("RNID_CitrinneBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_LapisBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_ラピス", 3)) then
		VariableEntry("G_所持_RNID_LapisBond_S_c", 1)
		AddBondRing("RNID_LapisBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_DiamantBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_ディアマンド", 3)) then
		VariableEntry("G_所持_RNID_DiamantBond_S_c", 1)
		AddBondRing("RNID_DiamantBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_AmberBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_アンバー", 3)) then
		VariableEntry("G_所持_RNID_AmberBond_S_c", 1)
		AddBondRing("RNID_AmberBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_JadeBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_ジェーデ", 3)) then
		VariableEntry("G_所持_RNID_JadeBond_S_c", 1)
		AddBondRing("RNID_JadeBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_IvyBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_アイビー", 3)) then
		VariableEntry("G_所持_RNID_IvyBond_S_c", 1)
		AddBondRing("RNID_IvyBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_KagetsuBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_カゲツ", 3)) then
		VariableEntry("G_所持_RNID_KagetsuBond_S_c", 1)
		AddBondRing("RNID_KagetsuBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_ZelkovBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_ゼルコバ", 3)) then
		VariableEntry("G_所持_RNID_ZelkovBond_S_c", 1)
		AddBondRing("RNID_ZelkovBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_FogadoBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_フォガート", 3)) then
		VariableEntry("G_所持_RNID_FogadoBond_S_c", 1)
		AddBondRing("RNID_FogadoBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_PandreoBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_パンドロ", 3)) then
		VariableEntry("G_所持_RNID_PandreoBond_S_c", 1)
		AddBondRing("RNID_PandreoBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_BonetBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_ボネ", 3)) then
		VariableEntry("G_所持_RNID_BonetBond_S_c", 1)
		AddBondRing("RNID_BonetBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_TimerraBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_ミスティラ", 3)) then
		VariableEntry("G_所持_RNID_TimerraBond_S_c", 1)
		AddBondRing("RNID_TimerraBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_PanetteBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_パネトネ", 3)) then
		VariableEntry("G_所持_RNID_PanetteBond_S_c", 1)
		AddBondRing("RNID_PanetteBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_MerrinBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_メリン", 3)) then
		VariableEntry("G_所持_RNID_MerrinBond_S_c", 1)
		AddBondRing("RNID_MerrinBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_HortensiaBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_オルテンシア", 3)) then
		VariableEntry("G_所持_RNID_HortensiaBond_S_c", 1)
		AddBondRing("RNID_HortensiaBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_SeadallBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_セアダス", 3)) then
		VariableEntry("G_所持_RNID_SeadallBond_S_c", 1)
		AddBondRing("RNID_SeadallBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_RosadoBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_ロサード", 3)) then
		VariableEntry("G_所持_RNID_RosadoBond_S_c", 1)
		AddBondRing("RNID_RosadoBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_MarigoldBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_ゴルドマリー", 3)) then
		VariableEntry("G_所持_RNID_MarigoldBond_S_c", 1)
		AddBondRing("RNID_MarigoldBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_LindonBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_リンデン", 3)) then
		VariableEntry("G_所持_RNID_LindonBond_S_c", 1)
		AddBondRing("RNID_LindonBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_SaphirBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_ザフィーア", 3)) then
		VariableEntry("G_所持_RNID_SaphirBond_S_c", 1)
		AddBondRing("RNID_SaphirBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_VeyleBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_ヴェイル", 3)) then
		VariableEntry("G_所持_RNID_VeyleBond_S_c", 1)
		AddBondRing("RNID_VeyleBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_MauvierBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_モーヴ", 3)) then
		VariableEntry("G_所持_RNID_MauvierBond_S_c", 1)
		AddBondRing("RNID_MauvierBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_AnnaBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_アンナ", 3)) then
		VariableEntry("G_所持_RNID_AnnaBond_S_c", 1)
		AddBondRing("RNID_AnnaBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_JeanBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_ジャン", 3)) then
		VariableEntry("G_所持_RNID_JeanBond_S_c", 1)
		AddBondRing("RNID_JeanBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_NelBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_エル", 3)) then
		VariableEntry("G_所持_RNID_NelBond_S_c", 1)
		AddBondRing("RNID_NelBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_RafalBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_ラファール", 3)) then
		VariableEntry("G_所持_RNID_RafalBond_S_c", 1)
		AddBondRing("RNID_RafalBond_S", 1)
	end
	
	if ( TryVariable("G_所持_RNID_ZelestiaBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_セレスティア", 3)) then
		VariableEntry("G_所持_RNID_ZelestiaBond_S_c", 1)
		AddBondRing("RNID_ZelestiaBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_GregoryBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_グレゴリー", 3)) then
		VariableEntry("G_所持_RNID_GregoryBond_S_c", 1)
		AddBondRing("RNID_GregoryBond_S", 1)
	end

	if ( TryVariable("G_所持_RNID_MadelineBond_S_c") == false ) and (CheckSupportRank("PID_リュール", "PID_マデリーン", 3)) then
		VariableEntry("G_所持_RNID_MadelineBond_S_c", 1)
		AddBondRing("RNID_MadelineBond_S", 1)
	end
end
