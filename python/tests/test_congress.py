"""Test Python bindings for Congress.gov API types."""
import os
import tempfile
from pathlib import Path

import pytest
from words_to_data import (
    Member, Party, Chamber,
    VotePosition,
    Dataset, DatasetMetadata,
    CongressClient, BillDownload,
)


def test_party_creation():
    """Test Party enum creation."""
    dem = Party.democrat()
    rep = Party.republican()
    ind = Party.independent()
    other = Party.other("Green")

    assert dem.is_democrat()
    assert rep.is_republican()
    assert ind.is_independent()
    assert other.name() == "Green"


def test_chamber_creation():
    """Test Chamber enum creation."""
    senate = Chamber.senate()
    house = Chamber.house()

    assert senate.is_senate()
    assert house.is_house()


def test_member_creation():
    """Test Member struct creation and access."""
    member = Member(
        bioguide_id="L000174",
        name="Patrick J. Leahy",
        first_name="Patrick",
        last_name="Leahy",
        party=Party.democrat(),
        state="VT",
        district=None,
        chamber=Chamber.senate(),
    )

    assert member.bioguide_id == "L000174"
    assert member.last_name == "Leahy"
    assert member.party.is_democrat()
    assert member.district is None
    assert member.chamber.is_senate()


def test_vote_position_creation():
    """Test VotePosition enum creation."""
    yea = VotePosition.yea()
    nay = VotePosition.nay()
    present = VotePosition.present()
    not_voting = VotePosition.not_voting()

    assert yea.is_yea()
    assert nay.is_nay()
    assert present.is_present()
    assert not_voting.is_not_voting()



def test_dataset_member_integration():
    """Test adding members to Dataset."""
    meta = DatasetMetadata(
        name="Test",
        description="Test dataset",
        author="Test",
        source_urls=[],
        license="MIT",
        version="1.0",
    )
    dataset = Dataset(meta)

    member = Member(
        bioguide_id="L000174",
        name="Patrick J. Leahy",
        first_name="Patrick",
        last_name="Leahy",
        party=Party.democrat(),
        state="VT",
        district=None,
        chamber=Chamber.senate(),
    )

    dataset.add_member(member)

    retrieved = dataset.get_member("L000174")
    assert retrieved is not None
    assert retrieved.last_name == "Leahy"



def test_congress_client_creation():
    """Test CongressClient can be created."""
    with tempfile.TemporaryDirectory() as tmpdir:
        client = CongressClient(api_key="test_key", cache_dir=tmpdir)
        assert client.api_key == "test_key"


def test_bill_download_creation():
    """Test BillDownload struct creation."""
    download = BillDownload(
        bill_id="119-hr-1",
        bill_xml="<xml>test</xml>",
        sponsors_json='{"bill":{}}',
        cosponsors_json='{"cosponsors":[]}',
        votes_json=None,
        member_jsons={},
    )

    assert download.bill_id == "119-hr-1"
    assert download.bill_xml == "<xml>test</xml>"
    assert download.votes_json is None


def test_dataset_load_bill_download():
    """Test loading BillDownload into Dataset."""
    # Get test data paths
    test_dir = Path(__file__).parent.parent.parent / "tests" / "test_data"
    bill_xml = (test_dir / "bills" / "119-hr-1.xml").read_text()
    member_json = (test_dir / "congress" / "members" / "L000174.json").read_text()

    download = BillDownload(
        bill_id="119-hr-1",
        bill_xml=bill_xml,
        sponsors_json='{"bill":{"sponsors":[{"bioguideId":"L000174"}]}}',
        cosponsors_json='{"cosponsors":[]}',
        votes_json=None,
        member_jsons={"L000174": member_json},
    )

    meta = DatasetMetadata(
        name="Test",
        description="Test",
        author="Test",
        source_urls=[],
        license="MIT",
        version="1.0",
    )
    dataset = Dataset(meta)

    bill_id = dataset.load_bill_download(download)

    assert bill_id == "119-hr-1"  # Uses bill_id from BillDownload
    assert dataset.get_bill(bill_id) is not None
    assert dataset.get_member("L000174") is not None
    assert dataset.get_sponsor_info(bill_id) is not None
